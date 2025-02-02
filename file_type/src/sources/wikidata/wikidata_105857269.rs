use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857269: FileFormat = FileFormat {
    id: 105_857_269,
    source_type: SourceType::Wikidata,
    name: "Horizon Project",
    extensions: &["hprj"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
