use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866103: FileFormat = FileFormat {
    id: 105_866_103,
    source_type: SourceType::Wikidata,
    name: "Microsoft PhoneBook",
    extensions: &["pbk"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
