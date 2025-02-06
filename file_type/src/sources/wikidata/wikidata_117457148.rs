use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117457148: FileFormat = FileFormat {
    id: 117_457_148,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Encrypted Database File 2.0",
    extensions: &["mda", "mdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
