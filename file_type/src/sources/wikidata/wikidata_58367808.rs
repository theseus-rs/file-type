use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58367808: FileFormat = FileFormat {
    id: 58_367_808,
    source_type: SourceType::Wikidata,
    name: "BSDIFF",
    extensions: &["bsdiff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
