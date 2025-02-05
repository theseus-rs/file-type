use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_15671948: FileFormat = FileFormat {
    id: 15_671_948,
    source_type: SourceType::Wikidata,
    name: "Blend file",
    extensions: &["blend"],
    media_types: &["application/x-blender"],
    signatures: &[],
    related_formats: &[],
};
