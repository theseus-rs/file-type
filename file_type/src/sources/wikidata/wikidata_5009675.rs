use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5009675: FileFormat = FileFormat {
    id: 5_009_675,
    source_type: SourceType::Wikidata,
    name: "CCP4",
    extensions: &["ccp4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
