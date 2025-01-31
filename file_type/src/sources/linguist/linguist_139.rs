use crate::format::FileFormat;

pub(crate) const LINGUIST_139: FileFormat = FileFormat {
    id: 139,
    puid: "linguist/139",
    name: "GraphQL",
    extensions: &["gql", "graphql", "graphqls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
