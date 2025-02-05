use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125823673: FileFormat = FileFormat {
    id: 125_823_673,
    source_type: SourceType::Wikidata,
    name: "Gzipped Tar File",
    extensions: &["tgz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
