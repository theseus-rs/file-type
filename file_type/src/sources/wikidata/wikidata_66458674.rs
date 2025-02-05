use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66458674: FileFormat = FileFormat {
    id: 66_458_674,
    source_type: SourceType::Wikidata,
    name: "Adobe Dimensions file format",
    extensions: &["dim"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
