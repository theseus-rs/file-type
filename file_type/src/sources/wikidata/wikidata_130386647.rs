use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130386647: FileFormat = FileFormat {
    id: 130_386_647,
    source_type: SourceType::Wikidata,
    name: "NuSMV file format",
    extensions: &["smv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
