use crate::format::FileFormat;

pub(crate) const LINGUIST_378: FileFormat = FileFormat {
    id: 378,
    puid: "linguist/378",
    name: "TypeScript",
    extensions: &["cts", "mts", "ts"],
    media_types: &["application/typescript"],
    internal_signatures: &[],
    related_formats: &[],
};
