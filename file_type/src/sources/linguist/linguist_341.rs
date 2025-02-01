use crate::format::FileFormat;

pub(crate) const LINGUIST_341: FileFormat = FileFormat {
    id: 341,
    puid: "linguist/341",
    name: "Scala",
    extensions: &["kojo", "sbt", "sc", "scala"],
    media_types: &["text/x-scala"],
    internal_signatures: &[],
    related_formats: &[],
};
