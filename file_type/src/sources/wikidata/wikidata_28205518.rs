use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205518: FileFormat = FileFormat {
    id: 28_205_518,
    source_type: SourceType::Wikidata,
    name: "atomix.scores",
    extensions: &["scores"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
