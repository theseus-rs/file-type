use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861120: FileFormat = FileFormat {
    id: 105_861_120,
    source_type: SourceType::Wikidata,
    name: "SyncTERM dialing directory",
    extensions: &["lst"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
