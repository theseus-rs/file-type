use crate::format::FileFormat;

pub(crate) const LINGUIST_274: FileFormat = FileFormat {
    id: 274,
    puid: "linguist/274",
    name: "PLpgSQL",
    extensions: &["pgsql", "sql"],
    media_types: &["text/x-sql"],
    internal_signatures: &[],
    related_formats: &[],
};
