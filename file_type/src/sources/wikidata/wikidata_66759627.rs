use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66759627: FileFormat = FileFormat {
    id: 66_759_627,
    source_type: SourceType::Wikidata,
    name: "Space-delimited formatted text file",
    extensions: &["prn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
