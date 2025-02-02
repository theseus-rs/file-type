use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28756196: FileFormat = FileFormat {
    id: 28_756_196,
    source_type: SourceType::Wikidata,
    name: "FWKCS NDX file",
    extensions: &["ndx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
