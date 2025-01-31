use crate::format::FileFormat;

pub(crate) const LINGUIST_202: FileFormat = FileFormat {
    id: 202,
    puid: "linguist/202",
    name: "Linker Script",
    extensions: &["ld", "lds", "x"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
