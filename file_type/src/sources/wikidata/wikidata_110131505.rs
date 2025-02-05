use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110131505: FileFormat = FileFormat {
    id: 110_131_505,
    source_type: SourceType::Wikidata,
    name: "Microsoft Publisher file format, version 2003",
    extensions: &["pub"],
    media_types: &["application/x-mspublisher"],
    signatures: &[],
    related_formats: &[],
};
