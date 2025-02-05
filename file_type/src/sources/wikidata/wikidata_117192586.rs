use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117192586: FileFormat = FileFormat {
    id: 117_192_586,
    source_type: SourceType::Wikidata,
    name: "Generic Encapsulated PostScript Graphic Format",
    extensions: &["ai3", "ai4", "ai5", "ai6", "ai7", "ai8"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
