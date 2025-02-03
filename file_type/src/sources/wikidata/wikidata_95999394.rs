use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95999394: FileFormat = FileFormat {
    id: 95_999_394,
    source_type: SourceType::Wikidata,
    name: "Formatted Checkpoint file",
    extensions: &["fchk"],
    media_types: &["chemical/x-gaussian-checkpoint"],
    internal_signatures: &[],
    related_formats: &[],
};
