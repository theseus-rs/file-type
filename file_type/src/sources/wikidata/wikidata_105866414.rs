use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866414: FileFormat = FileFormat {
    id: 105_866_414,
    source_type: SourceType::Wikidata,
    name: "Poser pose",
    extensions: &["pz2"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
