use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_15671948: FileFormat = FileFormat {
    id: 15_671_948,
    source_type: SourceType::Wikidata,
    name: "Blend file",
    extensions: &["blend"],
    media_types: &["application/x-blender"],
    internal_signatures: &[],
    related_formats: &[],
};
