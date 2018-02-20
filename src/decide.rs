/*! Decision procedures for Fungi type and effect system. */

use ast::*;
use bitype::{Ctx,HasClas,TypeError};
use std::fmt;
use std::rc::Rc;

/// Pair of related variables
pub type Var2 = (Var, Var);

/// Relational typing context: Relates pairs of variables, terms, etc
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub enum RelCtx {
    Empty,
    /// Define a name variable's sort
    NVar(RelCtxRec,Var,Var,Sort),
    /// Define an index variable's sort
    IVar(RelCtxRec,Var,Var,Sort),
    /// Define a type variable's kind
    TVar(RelCtxRec,Var,Var,Kind),
    /// Assume an index term equivalence, at a common sort
    Equiv(RelCtxRec,IdxTm,IdxTm,Sort),
    /// Assume an index term apartness, at a common sort
    Apart(RelCtxRec,IdxTm,IdxTm,Sort),
    /// Assume a proposition is true
    PropTrue(RelCtxRec,Prop),
}
pub type RelCtxRec = Rc<RelCtx>;

/// Convert the context into the corresponding relational context
pub fn relctx_of_ctx(c: &Ctx) -> RelCtx {
    // TODO
    unimplemented!()
}

/// Each relation has two sides, which we refer to as `L` and `R`
pub enum HandSide { L, R }

/// Convert the context into the corresponding relational context
pub fn ctx_of_relctx(c: &RelCtx, hs:HandSide) -> Ctx {
    // TODO
    unimplemented!()
}

/// Decision-related error
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub enum DecError {
    /// Type/sort/kind error during the decision procedure
    TypeError(TypeError),
}

/// Derivation for a decision procedure, expressed as deductive inference rules
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub struct Dec<Rule:HasClas> {
    pub ctx:RelCtx,
    pub rule:Rc<Rule>,
    pub clas:Rule::Clas,
    pub res:Result<bool,DecError>
}


/// Decide equivalence of two terms (types, indices, name terms)
pub mod equiv {
    use ast::*;
    use bitype::{Ctx,HasClas,TypeError};
    use std::fmt;
    use std::rc::Rc;
    use super::*;

    /// Name term equivalence rules
    #[derive(Clone,Debug,Eq,PartialEq,Hash)]
    pub enum NmTmRule {
        Refl(NameTm),
        Var(Var2),
        Bin(NmTmDec, NmTmDec),
        Lam(Var2,Sort,NmTmDec),
        App(NmTmDec, NmTmDec),
        NoParse(String),
    }
    pub type NmTmDec  = Dec<NmTmRule>;
    impl HasClas for NmTmRule {
        type Clas = Sort;
        fn tm_fam() -> String { "NmTm".to_string() }
    }
    
    /// Index term equivalence rules
    #[derive(Clone,Debug,Eq,PartialEq,Hash)]
    pub enum IdxTmRule {
        Var(Var2),
        Refl(IdxTm),
        Sing(NmTmDec),
        Empty,
        Disj(IdxTmDec, IdxTmDec),
        Union(IdxTmDec, IdxTmDec),
        Unit,
        Pair(IdxTmDec, IdxTmDec),
        Proj1(IdxTmDec),
        Proj2(IdxTmDec),
        Lam(Var2, Sort, IdxTmDec),
        App(IdxTmDec, IdxTmDec),
        Map(NmTmDec, IdxTmDec),
        FlatMap(IdxTmDec, IdxTmDec),
        Star(IdxTmDec, IdxTmDec),
        NoParse(String),
    }
    pub type IdxTmDec  = Dec<IdxTmRule>;
    impl HasClas for IdxTmRule {
        type Clas = Sort;
        fn tm_fam () -> String { "IdxTm".to_string() }
    }

    /// Value type equivalence rules
    #[derive(Clone,Debug,Eq,PartialEq,Hash)]
    pub enum TypeRule {
        Refl(Type),
        Var(Var2),
        Sum(TypeDec, TypeDec),
        Prod(TypeDec, TypeDec),
        Ref(IdxTm, TypeDec),
        Thk(IdxTm, CEffectDec),
        IdxApp(TypeDec, IdxTmDec),
        TypeApp(TypeDec, TypeDec),
        Nm(IdxTm),
        NmFn(NameTm),
        TypeFn(Var2, Kind, TypeDec),
        IdxFn(Var2, Sort, TypeDec),
        Rec(Var2, TypeDec),
        Exists(Var2, SortRec, Prop, TypeDec),
        NoParse(String),
    }
    pub type TypeDec  = Dec<TypeRule>;
    impl HasClas for TypeRule {
        type Clas = Kind;
        fn tm_fam() -> String { "Type".to_string() }
    }    

    /// Computation type equivalence rules
    #[derive(Clone,Debug,Eq,PartialEq,Hash)]
    pub enum CEffectRule {
        /// Every term is equal to itself
        Refl(CEffect),
    }
    pub type CEffectDec  = Dec<CEffectRule>;
    impl HasClas for CEffectRule {
        type Clas = Kind;
        fn tm_fam() -> String { "CEffect".to_string() }
    }    


    /// Decide if two name terms are equivalent under the given context
    pub fn decide_nmtm_equiv(ctx: &RelCtx, n:&NameTm, m:&NameTm, g:&Sort) -> NmTmDec {
        if n == m {
            return Dec{
                ctx:ctx.clone(),
                rule:Rc::new(NmTmRule::Refl(n.clone())),
                clas:g.clone(),
                res:Ok(true),
            }
        }
        else {        
            // TODO: the types are not identical, but could still be equivalent.
            // TODO: Use structural/deductive equiv rules.

            // NOTE #1: This is a priority to the extent that it is
            // used by name and index term _apartness_ checks, which
            // are likely to be the most important in many common
            // examples.
            unimplemented!()
        }
    }

    /// Decide if two index terms are equivalent under the given context
    pub fn decide_idxtm_equiv(ctx: &RelCtx, i:&IdxTm, j:&IdxTm, g:&Sort) -> IdxTmDec {
        if i == j {
            return Dec{
                ctx:ctx.clone(),
                rule:Rc::new(IdxTmRule::Refl(i.clone())),
                clas:g.clone(),
                res:Ok(true),
            }
        }
        else {        
            // TODO: the types are not identical, but could still be equivalent.
            // TODO: Use structural/deductive equiv rules.

            // NOTE #1: This is a priority to the extent that it is
            // used by name and index term _apartness_ checks, which
            // are likely to be the most important in many common
            // examples.
            unimplemented!()
        }
    }

    /// Decide if two type terms are equivalent under the given context
    pub fn decide_type_equiv(ctx: &RelCtx, a:&Type, b:&Type, k:&Kind) -> TypeDec {
        if a == b {
            return Dec{
                ctx:ctx.clone(),
                rule:Rc::new(TypeRule::Refl(a.clone())),
                clas:k.clone(),
                res:Ok(true),
            }
        }
        else {        
            // TODO: the types are not identical, but could still be
            // equivalent.  TODO: Use structural/deductive equiv
            // rules.

            // NOTE #2: This is a low priority over name and index
            // term _apartness_ checks, which are likely to be the
            // most important in many common examples.
            unimplemented!()
        }
    }


}


/// Decide apartness of two terms (indices, name terms)
pub mod apart {
    use ast::*;
    use bitype::{Ctx,HasClas,TypeError};
    use bitype;
    use std::fmt;
    use std::rc::Rc;
    use super::*;

    /// Name term apartness rules
    ///
    /// Fig. 24 of https://arxiv.org/abs/1610.00097v5
    ///
    /// Note: Removed D-Proj1 and D-Proj2, which are stale and should
    /// not be there in the rules.
    ///
    #[derive(Clone,Debug,Eq,PartialEq,Hash)]
    pub enum NmTmRule {
        Var(Var2),
        Sym(NmTmDec),
        Trans(equiv::NmTmDec, NmTmDec, NmTmDec),
        Bin1(NmTmDec),
        Bin2(NmTmDec),
        BinEq1(equiv::NmTmDec),
        BinEq2(equiv::NmTmDec),
        Lam(Var2,Sort,NmTmDec),
        App(NmTmDec, NmTmDec),
        Beta(bitype::NmTmDer, bitype::NmTmDer, NmTmDec),
        NoParse(String),
    }
    pub type NmTmDec  = Dec<NmTmRule>;
    impl HasClas for NmTmRule {
        type Clas = Sort;
        fn tm_fam() -> String { "NmTm".to_string() }
    }
    
    /// One side of an index term apartness
    ///
    /// Fig. 29 of https://arxiv.org/abs/1610.00097v5
    ///
    #[derive(Clone,Debug,Eq,PartialEq,Hash)]
    pub enum IdxTmRule {
        Var(Var2),
        Sym(IdxTmDec),
        Proj1(IdxTmDec,IdxTmDec),
        Proj2(IdxTmDec,IdxTmDec),
        Lam(Var2, Sort, IdxTmDec),
        App(IdxTmDec, equiv::IdxTmDec),
        Beta(IdxTmDec, bitype::IdxTmDer, bitype::IdxTmDer),
        Empty(NmTmDec),
        Single(IdxTmDec),
        Apart(IdxTmDec, IdxTmDec),
        Map(NmTmDec, IdxTmDec),
        FlatMap(IdxTmDec, IdxTmDec),
        Star(IdxTmDec, equiv::IdxTmDec),
        NoParse(String),
    }
    pub type IdxTmDec  = Dec<IdxTmRule>;
    impl HasClas for IdxTmRule {
        type Clas = Sort;
        fn tm_fam () -> String { "IdxTm".to_string() }
    }

    /// Decide if two name terms are apart under the given context
    pub fn decide_nmtm_apart(ctx: &RelCtx, n:&NameTm, m:&NameTm, g:&Sort) -> NmTmDec {
        // TODO: Use structural/deductive apartness rules.  Also, do
        // normalization of the name term (aka, beta reduction).
        unimplemented!()
    }

    /// Decide if two index terms are apart under the given context
    pub fn decide_idxtm_apart(ctx: &RelCtx, i:&IdxTm, j:&IdxTm, g:&Sort) -> IdxTmDec {
        // TODO: Use structural/deductive apartness rules. Also, do
        // normalization of the index term (aka, beta reduction).
        // Need to be careful not to expand Kleene star indefintely,
        // though. :)
        unimplemented!()
    }

}
