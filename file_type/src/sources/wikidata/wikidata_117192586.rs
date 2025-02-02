use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117192586: FileFormat = FileFormat {
    id: 117_192_586,
    source_type: SourceType::Wikidata,
    name: "Generic Encapsulated PostScript Graphic Format",
    extensions: &["ai3", "ai4", "ai5", "ai6", "ai7", "ai8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
