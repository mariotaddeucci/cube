use super::query_tools::QueryTools;
use super::sql_evaluator::MemberSymbol;
use super::sql_templates::PlanSqlTemplates;
use super::VisitorContext;
use cubenativeutils::CubeError;
use itertools::Itertools;
use std::rc::Rc;

pub trait BaseMember {
    fn to_sql(
        &self,
        context: Rc<VisitorContext>,
        templates: &PlanSqlTemplates,
    ) -> Result<String, CubeError>;
    fn alias_name(&self) -> String;
    fn member_evaluator(&self) -> Rc<MemberSymbol>;
    fn full_name(&self) -> String;
    fn as_base_member(self: Rc<Self>) -> Rc<dyn BaseMember>;
    fn cube_name(&self) -> &String;
    fn name(&self) -> &String;
    fn alias_suffix(&self) -> Option<String> {
        None
    }
}

pub struct BaseMemberHelper {}

impl BaseMemberHelper {
    pub fn upcast_vec_to_base_member<T: BaseMember>(vec: &Vec<Rc<T>>) -> Vec<Rc<dyn BaseMember>> {
        vec.iter()
            .map(|itm| itm.clone().as_base_member())
            .collect_vec()
    }

    pub fn iter_as_base_member<'a, T: BaseMember>(
        vec: &'a Vec<Rc<T>>,
    ) -> impl Iterator<Item = Rc<dyn BaseMember + 'static>> + 'a {
        vec.iter().map(|itm| itm.clone().as_base_member())
    }

    pub fn to_alias_vec(members: &Vec<Rc<dyn BaseMember>>) -> Vec<String> {
        members.iter().map(|m| m.alias_name()).collect_vec()
    }

    pub fn extract_symbols_from_members(
        members: &Vec<Rc<dyn BaseMember>>,
    ) -> Vec<Rc<MemberSymbol>> {
        members.iter().map(|m| m.member_evaluator()).collect_vec()
    }

    pub fn default_alias(
        cube_name: &String,
        member_name: &String,
        member_suffix: &Option<String>,
        query_tools: Rc<QueryTools>,
    ) -> Result<String, CubeError> {
        let cube_alias = query_tools.alias_for_cube(cube_name)?;
        Ok(PlanSqlTemplates::memeber_alias_name(
            &cube_alias,
            &member_name,
            member_suffix,
        ))
    }
}
