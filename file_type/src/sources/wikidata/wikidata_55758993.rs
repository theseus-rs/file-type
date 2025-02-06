use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55758993: FileFormat = FileFormat {
    id: 55_758_993,
    source_type: SourceType::Wikidata,
    name: "Microsoft Program Database file format, version 7",
    extensions: &["pdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
