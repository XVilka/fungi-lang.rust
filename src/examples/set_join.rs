#[test]
pub fn listing () { fgi_listing_test![
    decls {
        /// Optional natural numbers:
        type OpNat = (+ Unit + Nat );
        
        /// Levels (as numbers), for level trees.
        type Lev = ( Nat )
            
        /// Sequences (balanced binary level trees), whose leaves
        /// are optional natural numbers:
        type Seq = (
            rec seq. foralli (X,Y):NmSet.
                (+ (+ Unit + Nat)
                 + (exists (X1,X2,X3)   :NmSet | ((X1%X2%X3)=X:NmSet).
                    exists (Y1,Y2,Y3,Y4):NmSet | ((Y1%Y2%Y3%Y4)=Y:NmSet).
                    x Nm[X1] x Lev
                    x Ref[Y1](seq[X2][Y2])
                    x Ref[Y3](seq[X3][Y4]))
                )
        );                
            
        /// Sets (balanced binary hash tries), whose leaves are
        /// optional natural numbers:
        type Set = (
            rec set. foralli (X,Y):NmSet.
                (+ (+ Unit + Nat)
                 + (exists (X1,X2,X3)   :NmSet | ((X1%X2%X3)=X:NmSet).
                    exists (Y1,Y2,Y3,Y4):NmSet | ((Y1%Y2%Y3%Y4)=Y:NmSet).
                    x Nm[X1]
                    x Ref[Y1](set[X2][Y2])
                    x Ref[Y3](set[X3][Y4]))
                )
        );                

        /// Structural recursion over a binary tree (output names and pointers):
        idxtm    Bin   = (#x:Nm.         {x,@1} % {x,@2}  );
        idxtm    Bin1  = (#x:Nm.         {x,@1}           );
        idxtm    Bin2  = (#x:Nm.                  {x,@2}  );
        idxtm WS_Bin   = (#x:NmSet.{@!}( (Bin)  x         ));
        idxtm WS_Bin1  = (#x:NmSet.{@!}( (Bin1) x         ));
        idxtm WS_Bin2  = (#x:NmSet.{@!}( (Bin2) x         ));

        /// Trie_join: Index functions for output names and pointers:
        idxtm    Join  = (#x:NmSet.    ( (Bin)((Bin)^* x)));
        idxtm WS_Join  = (#x:NmSet.{@!}( {Join}  x       ));
    }
    
    let join:(
        Thk[0] foralli (X0, X1, X2, Y1, Y2):NmSet.
            0 Nm[X0] ->
            0 Set[X1][Y1] ->
            0 Set[X2][Y2] ->
        {
            {WS_Join} (X1%X2)
                ;
            Y1 % ( Y2 % (
                ( {WS_Join} (X0 % X1 % X2) )))
        }
        F Set
            [(Join)(X0 % X1 % X2)]
            [{WS_Join}(X0 % X1 % X2)]
    ) = {
        ret thunk fix join.
            #n. #set1. #set2.
        match set1 {
            on1 => { match set2 {
                on2  => {
                    let l :(Ref[{WS_Bin1} X0]Set[0][0]) = {
                        ref{n,(@1)}(roll inj1 on1)
                    }
                    let r :(Ref[{WS_Bin2} X0]Set[0][0]) = {
                        ref{n,(@2)}(roll inj1 on2)
                    }
                    ret roll inj2 pack (
                        X0, 0, 0,
                        ({WS_Bin1} X0), 0,
                        ({WS_Bin2} X0), 0,
                    )(n, l, r)
                }
                bin2 => {
                    unimplemented
                }
            }}
            bin1 => { match set2 {
                on2  => {
                    unimplemented
                }
                bin2 => {
                    // XXX
                    unimplemented
                }
            }}
        }    
    }

    ret 0
]}
/*
join : $\All{X{\disj}Y:\namesetsort}$
       $\SetNat{X} -> \SetNat{Y} -> \SetNat{\widehat{\textsf{join}}(X{\disj}Y)}$
    $|>~\widehat{\textsf{join}}(X{\disj}Y)$

join n l r = match (l,r) with
| SetLf(l), SetLf(r) => join_vecs n l r
| SetBin(_,_,_), SetLf(r) =>
             join n@1 l (split_vec n@2 r)
| SetLf(l), SetBin(_,_,_) =>
             join n@1 (split_vec n@2 l) r
| SetBin(ln,l0,l1), SetBin(rn,r0,r1) => 
 let (_,j0) = memo[ln@1](join ln@2 l0 r0)
 let (_,j1) = memo[rn@1](join rn@2 l1 r1)
 SetBin(n, j0, j1)
*/
