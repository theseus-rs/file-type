use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130386156: FileFormat = FileFormat {
    id: 130_386_156,
    puid: "wikidata/130386156",
    name: "Nit source code file",
    extensions: &["nit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
