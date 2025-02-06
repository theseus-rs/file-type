use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27684957: FileFormat = FileFormat {
    id: 27_684_957,
    source_type: SourceType::Wikidata,
    name: "Microsoft Publisher Pack and Go file format",
    extensions: &["puz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
