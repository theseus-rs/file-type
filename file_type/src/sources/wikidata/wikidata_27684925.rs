use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27684925: FileFormat = FileFormat {
    id: 27_684_925,
    source_type: SourceType::Wikidata,
    name: "Microsoft Publisher file format, version 14.0",
    extensions: &["pub"],
    media_types: &["application/vnd.ms-publisher"],
    signatures: &[],
    related_formats: &[],
};
