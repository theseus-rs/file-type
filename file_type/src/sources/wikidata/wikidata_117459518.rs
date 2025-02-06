use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117459518: FileFormat = FileFormat {
    id: 117_459_518,
    source_type: SourceType::Wikidata,
    name: "JWPUB",
    extensions: &["jwpub"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
