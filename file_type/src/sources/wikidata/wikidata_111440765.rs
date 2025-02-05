use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111440765: FileFormat = FileFormat {
    id: 111_440_765,
    source_type: SourceType::Wikidata,
    name: "Ruby source code",
    extensions: &["rb"],
    media_types: &["application/x-ruby", "text/x-ruby"],
    signatures: &[],
    related_formats: &[],
};
