use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95999881: FileFormat = FileFormat {
    id: 95_999_881,
    source_type: SourceType::Wikidata,
    name: "NDK seismographic data format",
    extensions: &["ndk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
