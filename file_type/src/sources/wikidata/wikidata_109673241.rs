use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109673241: FileFormat = FileFormat {
    id: 109_673_241,
    source_type: SourceType::Wikidata,
    name: "MS Outlook Express Email Index",
    extensions: &["idx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
