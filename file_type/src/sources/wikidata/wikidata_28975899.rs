use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975899: FileFormat = FileFormat {
    id: 28_975_899,
    source_type: SourceType::Wikidata,
    name: "MultiSurf geometry file",
    extensions: &["ms2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
