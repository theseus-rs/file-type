use crate::format::FileFormat;

pub(crate) const LINGUIST_256: FileFormat = FileFormat {
    id: 256,
    puid: "linguist/256",
    name: "ObjDump",
    extensions: &["objdump"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
