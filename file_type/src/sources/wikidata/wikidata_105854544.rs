use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854544: FileFormat = FileFormat {
    id: 105_854_544,
    source_type: SourceType::Wikidata,
    name: "Adobe Digital Editions Adobe Content Server Message",
    extensions: &["acsm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
