use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130386942: FileFormat = FileFormat {
    id: 130_386_942,
    puid: "wikidata/130386942",
    name: "objdump file format",
    extensions: &["objdump"],
    media_types: &["text/x-objdump"],
    internal_signatures: &[],
    related_formats: &[],
};
