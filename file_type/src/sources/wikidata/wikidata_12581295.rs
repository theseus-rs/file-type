use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_12581295: FileFormat = FileFormat {
    id: 12_581_295,
    source_type: SourceType::Wikidata,
    name: "KT 3G video file format",
    extensions: &["k3g"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
