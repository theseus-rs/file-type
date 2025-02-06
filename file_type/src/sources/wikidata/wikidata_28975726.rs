use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975726: FileFormat = FileFormat {
    id: 28_975_726,
    source_type: SourceType::Wikidata,
    name: "Gaussian Cube File",
    extensions: &["cub"],
    media_types: &["chemical/x-gaussian-cube"],
    signatures: &[],
    related_formats: &[],
};
