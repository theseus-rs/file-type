use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205619: FileFormat = FileFormat {
    id: 28_205_619,
    source_type: SourceType::Wikidata,
    name: "RIPscrip version 2 Hot Icon",
    extensions: &["bmh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
