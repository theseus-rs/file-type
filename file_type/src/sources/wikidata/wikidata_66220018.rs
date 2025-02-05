use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66220018: FileFormat = FileFormat {
    id: 66_220_018,
    source_type: SourceType::Wikidata,
    name: "Adobe GoLive Actions file format",
    extensions: &["actions"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
