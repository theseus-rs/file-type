use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27684843: FileFormat = FileFormat {
    id: 27_684_843,
    source_type: SourceType::Wikidata,
    name: "Microsoft Publisher file format, version 2.0",
    extensions: &["pub"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
