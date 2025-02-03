use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866414: FileFormat = FileFormat {
    id: 105_866_414,
    source_type: SourceType::Wikidata,
    name: "Poser pose",
    extensions: &["pz2"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
