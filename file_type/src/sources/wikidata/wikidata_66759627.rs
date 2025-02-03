use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66759627: FileFormat = FileFormat {
    id: 66_759_627,
    source_type: SourceType::Wikidata,
    name: "Space-delimited formatted text file",
    extensions: &["prn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
