公開 設計図 sampleExtends 継承 SomeParent {
	
	公開 静的 無 開始(文字列 引数[]){
		親に連絡(引数);
		
		整数 カウント = 10;
		
		整数 合計 = 0;

		各(整数 i = 0; i < カウント; i++ ){
			合計 += i;
		}
		システム.出力.書き込む("Sum of 10: " + 合計);
	}
}