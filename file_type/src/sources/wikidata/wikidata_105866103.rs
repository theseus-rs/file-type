use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866103: FileFormat = FileFormat {
    id: 105_866_103,
    source_type: SourceType::Wikidata,
    name: "Microsoft PhoneBook",
    extensions: &["pbk"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
