use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857269: FileFormat = FileFormat {
    id: 105_857_269,
    source_type: SourceType::Wikidata,
    name: "Horizon Project",
    extensions: &["hprj"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
