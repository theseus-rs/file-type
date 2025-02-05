use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27684816: FileFormat = FileFormat {
    id: 27_684_816,
    source_type: SourceType::Wikidata,
    name: "Microsoft Publisher file format, version 1.0",
    extensions: &["pub"],
    media_types: &["application/vnd.ms-publisher", "application/x-mspublisher"],
    signatures: &[],
    related_formats: &[],
};
