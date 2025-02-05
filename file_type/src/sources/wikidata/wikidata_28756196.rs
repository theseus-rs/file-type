use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28756196: FileFormat = FileFormat {
    id: 28_756_196,
    source_type: SourceType::Wikidata,
    name: "FWKCS NDX file",
    extensions: &["ndx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
