use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110133975: FileFormat = FileFormat {
    id: 110_133_975,
    source_type: SourceType::Wikidata,
    name: "Microsoft Publisher file format, version 2013",
    extensions: &["pub"],
    media_types: &["application/x-mspublisher"],
    internal_signatures: &[],
    related_formats: &[],
};
