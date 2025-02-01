use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_95999394: FileFormat = FileFormat {
    id: 95_999_394,
    puid: "wikidata/95999394",
    name: "Formatted Checkpoint file",
    extensions: &["fchk"],
    media_types: &["chemical/x-gaussian-checkpoint"],
    internal_signatures: &[],
    related_formats: &[],
};
