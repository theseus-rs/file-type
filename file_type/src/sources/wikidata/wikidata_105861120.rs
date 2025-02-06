use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861120: FileFormat = FileFormat {
    id: 105_861_120,
    source_type: SourceType::Wikidata,
    name: "SyncTERM dialing directory",
    extensions: &["lst"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
