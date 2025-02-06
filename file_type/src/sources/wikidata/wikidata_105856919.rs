use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856919: FileFormat = FileFormat {
    id: 105_856_919,
    source_type: SourceType::Wikidata,
    name: "Google Desktop Gadget manifest",
    extensions: &["gmanifest"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
