use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205896: FileFormat = FileFormat {
    id: 28_205_896,
    source_type: SourceType::Wikidata,
    name: "DESR VFF",
    extensions: &["vff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
