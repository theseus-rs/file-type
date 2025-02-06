use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110134612: FileFormat = FileFormat {
    id: 110_134_612,
    source_type: SourceType::Wikidata,
    name: "Microsoft Publisher file format, version 2016",
    extensions: &["pub"],
    media_types: &["application/x-mspublisher"],
    signatures: &[],
    related_formats: &[],
};
