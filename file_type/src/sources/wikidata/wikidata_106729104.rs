use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_106729104: FileFormat = FileFormat {
    id: 106_729_104,
    source_type: SourceType::Wikidata,
    name: "mz5",
    extensions: &["mz5"],
    media_types: &["application/x-hdf5"],
    signatures: &[],
    related_formats: &[],
};
