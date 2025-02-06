use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_70357595: FileFormat = FileFormat {
    id: 70_357_595,
    source_type: SourceType::Wikidata,
    name: "Jupyter notebook file",
    extensions: &["ipynb"],
    media_types: &["application/x-ipynb+json"],
    signatures: &[],
    related_formats: &[],
};
